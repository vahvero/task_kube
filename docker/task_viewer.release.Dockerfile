FROM trion/ng-cli:20.3.3 AS angular-builder

WORKDIR /task_viewer

COPY --chmod=777 task_viewer .

RUN npm install && ng build

FROM nginx:stable-alpine

COPY --from=angular-builder /task_viewer/dist/task_viewer/browser /usr/share/nginx/html
COPY nginx/nginx.release.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
