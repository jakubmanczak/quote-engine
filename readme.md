# Quote Engine

Quote Engine is a web based quote aggregation system, helping you collect and organize "quotes" from those around you.
Written as a replacement for a `#quotes` channel on a chat service, it's a digital notebook where a class might capture their professors' funniest remarks.

## Deploying

It's easiest to deploy Quote Engine via docker compose. In order to do so, clone the repo and create a `.env` file in the same directory as
the provided `docker-compose.yml`. A structure similar to hereafter is expected, where:

- `NEXT_PUBLIC_SERVER_PATH` is the path to your backend API root, with the trailing slash removed;
- `SECRET` is a chain of characters used as a key in the authentication of users;
- `PORT` specifies where the backend will be listening.

```env
NEXT_PUBLIC_SERVER_PATH=https://your.deploymen.tt/api
SECRET=RWLYSONOSTARPOZNANCDNSBCD4L52SPM
PORT=2019
```

Then, simply run `docker compose build` and `docker compose up`.

A default account (login: `admin`; password: `admin`) will be created. You are strongly encouraged to change these credentials after setup.
