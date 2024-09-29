# Fast File Sharing

Эта утилита позволяет размещать большие файлы для встраивания в Notion, обходя ограничения бесплатной версии. Вместо загрузки файлов в Notion этот инструмент размещает файлы на вашем сервере и генерирует ссылки, которые можно вставить в страницы Notion. Файлы могут быть фотографиями, видео или другими мультимедийными данными, как если бы они были загружены напрямую в Notion.

## Настройка сервера

### Шаг 1: Создание директории для хранения файлов

Создайте директорию, которая будет использоваться для хранения файлов:

```bash
sudo mkdir -p /var/www/sharedfiles
```

Установите правильные права доступа для владельца и группы, чтобы NGINX и пользователь, загружающий файлы, могли обращаться к директории:

```bash
sudo chown -R www-data:www-data /var/www/sharedfiles
sudo chmod -R 775 /var/www/sharedfiles
```

### Шаг 2: Настройка setgid для автоматического назначения прав группы

Убедитесь, что новые файлы и папки, добавленные в директорию, автоматически наследуют группу `www-data`:

```bash
sudo chmod g+s /var/www/sharedfiles
```

### Шаг 3: Конфигурация Nginx для хранения файлов

Создайте новый файл конфигурации Nginx, например, `/etc/nginx/sites-available/file_sharing_dynamic.conf`:

```bash
sudo nano /etc/nginx/sites-available/file_sharing_dynamic.conf
```

Добавьте следующую конфигурацию:

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

Создайте символическую ссылку для активации конфигурации:

```bash
sudo ln -s /etc/nginx/sites-available/file_sharing_dynamic.conf /etc/nginx/sites-enabled/
```

Проверьте конфигурацию Nginx и перезагрузите его:

```bash
sudo nginx -t
sudo systemctl reload nginx
```

### Шаг 4: Создание пользователя для загрузки файлов

Создайте пользователя, который будет использоваться для загрузки файлов:

```bash
sudo adduser sharefilesuser
```

Установите пароль для пользователя, следуя инструкциям. Затем предоставьте этому пользователю права на директорию:

```bash
sudo chown -R sharefilesuser:www-data /var/www/sharedfiles
sudo chmod -R 775 /var/www/sharedfiles
```

Примените бит setgid, чтобы все файлы, созданные этим пользователем, автоматически получали правильные права:

```bash
sudo chmod g+s /var/www/sharedfiles
```

### Шаг 5: Установка утилиты для работы с паролями и создание пользователя для аутентификации

Чтобы включить базовую аутентификацию для загрузки файлов, установите утилиту для управления паролями:

```bash
sudo apt-get install apache2-utils
```

Затем создайте файл с именем пользователя и паролем для аутентификации:

```bash
htpasswd -c /etc/nginx/.htpasswd sharefilesuser
```

Вы будете запрошены на ввод пароля для учетной записи `sharefilesuser`. Этот пароль потребуется для загрузки файлов на сервер.

## Создание файла Config.toml

Используйте файл [Config.example.toml](Config.example.toml) как шаблон. В файле укажите:

- Путь к директории хранения файлов
- Имя пользователя и пароль
- Адрес сервера
- Шаблон ссылки с секретным ключом

## Сборка и запуск проекта

- В директории проекта выполните следующую команду для сборки проекта:
    ```bash
    cargo build --release
    ```

- Переместите [исполняемый файл приложения](target%2Frelease%2Fupload_to_server.exe) в постоянное место.

- Запустите его один раз.

Теперь появится пункт меню "Загрузить файл на сервер", который загрузит файл и скопирует ссылку в буфер обмена.