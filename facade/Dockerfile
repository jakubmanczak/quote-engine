FROM node:20
WORKDIR /facade
COPY . .
RUN npm install
RUN npx next build
CMD npx next start
