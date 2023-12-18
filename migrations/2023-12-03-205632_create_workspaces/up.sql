CREATE TABLE workspaces (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name varchar(250) not null,
    created_by uuid not null references users (id),
    created_at timestamptz default current_timestamp not null,
    modified_at timestamptz default current_timestamp not null
);

CREATE FUNCTION update_workspaces_modified_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.modified_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER workspaces_mod_datetime
    BEFORE UPDATE ON workspaces
    FOR EACH ROW
    EXECUTE PROCEDURE update_workspaces_modified_at();
