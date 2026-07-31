import { Component, computed, effect, inject } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { TaskService } from './task-service';
import { Task } from './task/task';
import { CommonModule } from '@angular/common';
import { rxResource } from '@angular/core/rxjs-interop';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, Task, CommonModule],
  templateUrl: './app.html',
  styleUrl: './app.scss',
})
export class App {
  private readonly taskService = inject(TaskService);

  taskResource = rxResource({ stream: () => this.taskService.getTaskSse() })
  tasks = computed(() => this.taskResource.value() ?? []);
  inprogressCount = computed(() => this.tasks().filter((x) => x.state === "IN-PROGRESS").length);
  uncompleted_tasks = computed(() => this.tasks().filter((x) => x.state !== "COMPLETED").sort(
    (a, b) => {
      if (a.state === b.state) return 0;
      if (a.state === "PENDING") return 1;
      return -1;
    }
  ))

  createTask() {
    this.taskService.createTask().subscribe(() => {
      console.log('Task created');
    });
  }

  resetTasks() {
    this.taskService.resetTasks().subscribe(
      () => { console.log("Tasks reset"); }
    )
  }
}
