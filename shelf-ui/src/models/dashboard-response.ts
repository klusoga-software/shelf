export interface DashboardResponse {
  id: string;
  rowSpan: number | undefined;
  columnSpan: number | undefined;
  columnOffset: [columns: number];
}
