import { SidebarTrigger } from "./ui/sidebar";

export const PageHeader = ({ title }: { title: string }) => (
  <div className="flex items-center py-3">
    <SidebarTrigger />
    <h1 className="text-xl">{title}</h1>
  </div>
);
