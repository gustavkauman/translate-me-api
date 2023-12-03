CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    username varchar(100) not null,
    name varchar(250) not null,
    mail varchar(250) not null,
    created_at timestamptz default current_timestamp not null,
    modified_at timestamptz default current_timestamp not null,
    UNIQUE(username),
    UNIQUE(mail)
);

CREATE FUNCTION update_users_modified_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.modified_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER users_mod_datetime
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE PROCEDURE update_users_modified_at();

