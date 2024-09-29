# Fast File Sharing

This utility allows you to host large files for embedding in Notion, bypassing limitations of the free version. Instead of uploading files to Notion, this tool hosts the files on your server and generates links that you can insert into Notion pages. The files can be photos, videos, or any other media, seamlessly appearing as though they were uploaded directly to Notion.

## Server Setup

### Step 1: Creating a Directory for File Storage

Create a directory that will be used to store files:

```bash
sudo mkdir -p /var/www/sharedfiles
```

Set the correct owner and group permissions to ensure NGINX and your upload user can access the files:

```bash
sudo chown -R www-data:www-data /var/www/sharedfiles
sudo chmod -R 775 /var/www/sharedfiles
```

### Step 2: Setting Up setgid for Automatic Group Permissions

Ensure that new files and folders added to the directory automatically inherit the `www-data` group:

```bash
sudo chmod g+s /var/www/sharedfiles
```

### Step 3: Configuring Nginx for File Storage

Create a new Nginx configuration file, for example, `/etc/nginx/sites-available/file_sharing_dynamic.conf`:

```bash
sudo nano /etc/nginx/sites-available/file_sharing_dynamic.conf
```

Add the following configuration:

```nginx
server {
    listen 80;
    server_name 85.198.110.190;

    location /files/ {
        set $valid_key "your_secret_key";

        if ($arg_key != $valid_key) {
            return 403;
        }

        alias /var/www/sharedfiles/;
        types {
            video/mp4 mp4;
            video/webm webm;
            video/ogg ogg;
        }

        add_header Accept-Ranges bytes;
        add_header 'Access-Control-Allow-Origin' '*';
        add_header 'Access-Control-Allow-Methods' 'GET, HEAD, OPTIONS';
        add_header 'Access-Control-Allow-Headers' 'Origin, X-Requested-With, Content-Type, Accept';

        default_type application/octet-stream;
    }    
}


server {
    listen 8080;
    server_name 85.198.110.190;

    location /upload_files/ {
        dav_methods PUT DELETE MKCOL COPY MOVE;
        client_body_temp_path /tmp/incoming;
        alias /var/www/sharedfiles/;

        create_full_put_path on;
        dav_access user:rw group:rw all:r;

        client_body_buffer_size 128k;
        client_max_body_size 500M;

        client_body_in_file_only clean;
        default_type application/octet-stream;

        auth_basic "Restricted Upload Area";
        auth_basic_user_file /etc/nginx/.htpasswd;
    }
}
```

Create a symbolic link to activate the configuration:

```bash
sudo ln -s /etc/nginx/sites-available/file_sharing_dynamic.conf /etc/nginx/sites-enabled/
```

Test the Nginx configuration and reload it:

```bash
sudo nginx -t
sudo systemctl reload nginx
```

### Step 4: Creating a User for File Uploads

Create a user that will be used for uploading files:

```bash
sudo adduser sharefilesuser
```

Set a password for the user by following the instructions. Then, grant this user permissions for the directory:

```bash
sudo chown -R sharefilesuser:www-data /var/www/sharedfiles
sudo chmod -R 775 /var/www/sharedfiles
```

Apply the setgid bit so that all files created by this user automatically get the correct permissions:

```bash
sudo chmod g+s /var/www/sharedfiles
```

### Step 5: Installing Password Utility and Creating a User for Authentication

To enable basic authentication for file uploads, you'll need to install a utility to manage passwords:

```bash
sudo apt-get install apache2-utils
```

Then, create a password file with a username and password for authentication:

```bash
htpasswd -c /etc/nginx/.htpasswd sharefilesuser
```

You will be prompted to enter a password for the `sharefilesuser` account. This password will be required when uploading files to the server.

## Creating the Config.toml File

Use the [Config.example.toml](Config.example.toml) file as a template. In the file, specify:

- The path to the file storage directory
- Username and password
- Server address
- Link template with a secret key

## Building and Running the Project

- In the project directory, run the following command to build the project:
    ```bash
    cargo build --release
    ```

- Move the [application executable](target%2Frelease%2Fupload_to_server.exe) to a permanent location.

- Run it once.

Now, there is a menu item called "Upload File to Server," which will upload the file and copy the link to the clipboard.