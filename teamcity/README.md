# teamcity-playground

TeamCity playground with docker compose.

## Usage

Use `docker-compose up` to start TeamCity Server and Agent. After a while, it
will be running at [http://localhost:8111/](http://localhost:8111/).

## Data folder

All mounted volumes are stored in the `data` folder, which is ignored.

Tip: symlink the data folder to Google Drive to keep the data across multiple
computers.

```
ln -s ../../Google Drive/teamcity-playground/data .
```

or

```
mklink /D data ..\..\Google Drive\teamcity-playground\data
```
