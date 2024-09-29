# Fast File Sharing

This utility is designed to host large files in the free version of Notion. When importing pages, Notion doesn’t upload the files but uses links to the original files. With this utility, you can insert links to photos, videos, or any other files as if the files were uploaded directly to Notion. The utility allows you to quickly upload these files to your server.

## Server Setup

### Step 1: Creating a Directory for File Storage

Create a new directory that will be used to store files:

```bash
sudo mkdir -p /var/www/sharedfiles
```

Then, set the owner and group for this directory so that files inherit the correct permissions automatically:

```bash
sudo chown -R www-data:www-data /var/www/sharedfiles
```

Set appropriate read and write permissions:

```bash
sudo chmod -R 755 /var/www/sharedfiles
```

### Step 2: Setting Up setgid for Automatic Group Permissions

To ensure that new files and folders added to the directory automatically inherit the `www-data` group, set the setgid bit:

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
    server_name your_domain_or_ip;

    location /files/ {
        set $valid_key "your_secret_key";

        if ($arg_key != $valid_key) {
            return 403;
        }

        alias /var/www/sharedfiles/;
        autoindex on;
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
sudo chown -R sharefilesuser:www-data /var/www/nikita_to_notion
sudo chmod -R 775 /var/www/nikita_to_notion
```

Apply the setgid bit so that all files created by this user automatically get the correct permissions:

```bash
sudo chmod g+s /var/www/nikita_to_notion
```

## Creating the Config.toml File

Use the [Config.example.toml](Config.example.toml) file as a template. In the file, you should specify:

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