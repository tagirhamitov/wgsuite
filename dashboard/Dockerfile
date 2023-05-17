FROM node:20-alpine as builder

WORKDIR /app

COPY . .

RUN npm install --only=prod
RUN npm run build

FROM nginx:1.24.0-alpine

COPY --from=builder /app/build /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
