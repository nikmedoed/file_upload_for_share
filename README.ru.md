# Fast file sharing

Эта утилита создана для размещения больших файлов в бесплатной версии Notion. При импорте страниц, Notion не загружает файлы, а использует ссылки на оригинальные файлы. Благодаря этой утилите вы можете вставить ссылку на фото, видео или любой другой файл, будто этот файл загружен напрямую в Notion. Утилита позволяет быстро загружать такие файлы на ваш сервер.

## Настройка сервера

### Шаг 1. Создание директории для хранения файлов

Создайте новую директорию, которая будет использована для хранения файлов:

```bash
sudo mkdir -p /var/www/sharedfiles
```

Затем установите владельца и группу для этой директории, чтобы файлы автоматически наследовали правильные права:

```bash
sudo chown -R www-data:www-data /var/www/sharedfiles
```

Настройте права доступа для чтения и записи:

```bash
sudo chmod -R 755 /var/www/sharedfiles
```

### Шаг 2. Настройка setgid для автоматических прав группы

Для того чтобы новые файлы и папки, добавленные в директорию, автоматически получали группу `www-data`, настройте специальный бит setgid:

```bash
sudo chmod g+s /var/www/sharedfiles
```

### Шаг 3. Настройка Nginx для файлового хранилища

Создайте новый файл конфигурации Nginx, например, `/etc/nginx/sites-available/file_sharing_dynamic.conf`:

```bash
sudo nano /etc/nginx/sites-available/file_sharing_dynamic.conf
```

Добавьте следующую конфигурацию:

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

Создайте символическую ссылку для активации конфигурации:

```bash
sudo ln -s /etc/nginx/sites-available/file_sharing_dynamic.conf /etc/nginx/sites-enabled/
```

Проверьте конфигурацию Nginx и перезагрузите его:

```bash
sudo nginx -t
sudo systemctl reload nginx
```

### Шаг 4. Создание пользователя для загрузки файлов

Создайте пользователя, от имени которого будут загружаться файлы:

```bash
sudo adduser sharefilesuser
```

Установите пароль для пользователя, следуя инструкциям. Далее, предоставьте этому пользователю права на директорию:

```bash
sudo chown -R sharefilesuser:www-data /var/www/nikita_to_notion
sudo chmod -R 775 /var/www/nikita_to_notion
```

Примените бит setgid, чтобы все файлы, созданные этим пользователем, автоматически получали правильные права:

```bash
sudo chmod g+s /var/www/nikita_to_notion
```

## Создание файла Config.toml

Используйте файл [Config.example.toml](Config.example.toml) в качестве примера. В файле нужно указать:

- Путь до директории для хранения файлов
- Пользователя и пароль
- Адрес сервера
- Шаблон ссылки с секретным ключом

## Сборка и запуск проекта
- В директории проекта
    ```cargo build --release```
- Переместите [exe файл приложения](target%2Frelease%2Fupload_to_server.exe) в постоянное место хранения
- Запустите один раз
Теперь в меню есть пункт "Отправить файл на сервер", который загрузит и скопирует ссылку в буфер обмена.