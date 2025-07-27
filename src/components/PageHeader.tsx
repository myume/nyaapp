import { Button } from "./ui/button";
import { SidebarTrigger } from "./ui/sidebar";

export const PageHeader = ({
  title,
  onClick,
}: {
  title: string;
  onClick?: () => void;
}) => (
  <div className="flex items-center py-3">
    <SidebarTrigger />
    {onClick ? (
      <Button
        variant="ghost"
        className="cursor-pointer h-full p-1"
        onClick={onClick}
      >
        <h1 className="text-xl">{title}</h1>
      </Button>
    ) : (
      <h1 className="text-xl p-1">{title}</h1>
    )}
  </div>
);
