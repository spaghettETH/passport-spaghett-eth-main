# Dockerfile
FROM node:18

# create destination directory
RUN mkdir -p /usr/src/passport-app
WORKDIR /usr/src/passport-app

# copy the app, note .dockerignore
COPY . /usr/src/passport-app/

RUN npm install -g npm
RUN yarn
RUN yarn build

EXPOSE 3000

ENV NUXT_HOST=0.0.0.0
ENV NUXT_PORT=3000

CMD [ "npm", "start" ]