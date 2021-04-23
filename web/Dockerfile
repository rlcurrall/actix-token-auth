### < BUILD > ###
FROM node:lts-alpine AS build
ENV NODE_ENV production
WORKDIR /app
COPY .env.example .env
COPY package*.json ./
RUN npm install
COPY . .
RUN npm run build
### < BUILD > ###

### < PRODUCTION > ###
FROM nginx:stable-alpine as prod
COPY --from=build-stage /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
### < PRODUCTION > ###
