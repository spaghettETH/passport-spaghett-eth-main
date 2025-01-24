# Dockerfile
FROM node:18

# create destination directory
RUN mkdir -p /usr/src/bartobe-serve
WORKDIR /usr/src/bartobe-serve

# copy the app, note .dockerignore
COPY . /usr/src/bartobe-serve/

RUN npm install -g npm
RUN yarn
RUN yarn build

EXPOSE 3000

CMD [ "yarn", "serve" ]