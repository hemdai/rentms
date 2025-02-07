FROM node:20 as build-stage
WORKDIR /app
COPY package.json .
COPY package-lock.json .
RUN npm install
COPY . .
EXPOSE 3000

#Staging
FROM build-stage as rentms-frontend-staging-stage
RUN npm install -g http-server
COPY --from=build-stage /app .
RUN npm run build_staging

#Production
FROM build-stage as rentms-frontent-prod-stage
RUN npm install -g http-server
COPY --from=build-stage /app .
RUN npm run build_production