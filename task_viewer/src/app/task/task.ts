import { Component, input } from '@angular/core';
import { TaskState } from '../itask';

@Component({
  selector: 'app-task',
  imports: [],
  templateUrl: './task.html',
  styleUrl: './task.scss',
})
export class Task {
  state = input.required<TaskState>();
  id = input.required<number>();
}
