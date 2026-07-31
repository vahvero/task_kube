import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { ITask } from './itask';
import { fromEvent, Observable, tap, map } from 'rxjs';


@Injectable({
  providedIn: 'root',
})
export class TaskService {
  private readonly tasksUrl = 'api/tasks';
  private readonly client = inject(HttpClient);
  private readonly eventSource = new EventSource(this.tasksUrl);

  getTasks(): Observable<ITask[]> {
    return this.client.get<ITask[]>(this.tasksUrl);
  }

  getTaskSse(): Observable<ITask[]> {
    return fromEvent<MessageEvent>(this.eventSource, 'message').pipe(
      map((event) => {
        return JSON.parse(event.data);
      }),
      tap((event) => console.log("SSE length", event.length)),
      tap((event) => console.log("SSE in-progress", event.filter((x) => x.state === "IN-PROGRESS")))
    )
  }

  createTask(): Observable<string> {
    return this.client.post<string>('api/task', {});
  }

  resetTasks() {
    return this.client.post<string>("api/reset", {});
  }
}
