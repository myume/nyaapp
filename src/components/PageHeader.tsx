import { SidebarTrigger } from "./ui/sidebar";

export const PageHeader = ({ title }: { title: String }) => (
  <div className="flex items-center">
    <SidebarTrigger />
    <h1 className="text-lg">{title}</h1>
  </div>
);
