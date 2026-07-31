export interface ITask {
  id: number;
  description: string;
  delay: number;
  state: TaskState;
}

export type TaskState = 'PENDING' | 'IN-PROGRESS' | 'COMPLETED';
