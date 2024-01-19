# Art Event Finder

## Overview

Art Event Finder is a dynamic platform designed to connect art enthusiasts with a wide range of art events. This project aims to make it easier for people to discover and engage with art events in their region, which are often challenging to find due to a lack of centralized information. The platform serves as a bridge between artists, art groups, and the community, enhancing the visibility of local art events and fostering a vibrant art scene.

## Target Audience

- **Art Enthusiasts**: Individuals who are passionate about experiencing and supporting various forms of art, including music, theater, dance, and visual arts.
- **Artists and Art Groups**: Creators seeking to promote their events, reach a broader audience, and engage more effectively with their fans.
- **Art Event Organizers**: Professionals and organizations that organize art-related events and seek a platform for promotion and audience engagement.

## Features

1. **User and Artist Accounts**: Separate account types for regular users and artists, with customized features for each.
2. **Event Discovery**: Users can discover art events based on preferences such as location, art category, and artist.
3. **Social Media Integration**: Automatic event creation through monitoring artists' social media channels, particularly Instagram.
4. **Follow System**: Users can follow their favorite artists to get updates and notifications about upcoming events.
5. **Admin Moderation**: Admins can approve or reject artist accounts and event postings, ensuring content quality and reliability.
6. **Event Source Tracking**: Users can suggest new social media accounts for event source tracking, subject to admin approval.

## Technologies

- Backend API developed using [Your Chosen Technology Stack]
- Integration with Keycloak for authentication and authorization
- Utilization of message queues for background processing and notifications
- Leveraging external APIs for social media integration and event data extraction

## Running the project

### Prerequisites
- [Docker](https://docs.docker.com/desktop/?_gl=1*ah4slm*_ga*MTYzNTIzOTQ5Mi4xNzAzMjkyOTU3*_ga_XJWPQMJYHQ*MTcwMzM2OTc2My4zLjEuMTcwMzM2OTc2OC41NS4wLjA.)
- [Docker Compose](https://docs.docker.com/compose/install/)

Configure your envs files. `.env` is used by docker compose and `config.yml` is used by the server
```
cp .env.example .env
cp config.yml.example config.yml
```

Build web server image and initialize docker containers

```
./scripts/build_and_start_docker.sh
```

If it's your first time running the project, create a new Realm on KeyCloak
- Go to `127.0.0.1:8081`
- Login to _Administration Console_ using username and password that are configured inside `.env`
- Click at _Create realm_ at the dropdown menu on the top left
- Import `events_finder_realm.json` and create it

## Development

### Prerequisites
- [Clippy](https://github.com/rust-lang/rust-clippy?tab=readme-ov-file#usage)

### Git hooks

Configure git pre-push hook to make sure the code is valid before pushing it

```
ln -s ../../scripts/pre-push.sh .git/hooks/pre-push
```

### Configuration details

#### Server

- address: Address where the server is connected to
- port: Port where the server is connected to

#### Keycloak

- keycloak_realm: Target keycloak realm. Use config.yml.example value if you're using `events_finder_realm.json` to create the realm
- keycloak_client_id: Client it used by the server to connect to keycloak. Use config.yml.example value if you're using `events_finder_realm.json` to create the realm
- keycloak_client_secret: Client secret required to use the client. To get it do the following steps:
  - Go to Keycloak admin console
  - Select the realm configured by `keycloak_realm` config
  - At _Clients_ menu, select the client configured by `keycloak_client_id` config
  - At _Credentials_ tab, you'll find the _Client Secret_
  - Replace `<client-secret>` by the secret
- keycloak_base_url: Keycloak's address. Use config.yml.example value if you're running keycloak configured at docker-compose.yml
- keycloak_jwt_public_key: Keycloak realm's RS256 public key. To get it do the following steps:
  - Go to Keycloak admin console
  - Select the realm configured by `keycloak_realm` config
  - At _Realm settings" menu, go to _Keys_ tab
  - Get the RS256 public key
  - Replace `<jwt-public-key>` by the key
