import { PageHeader } from "@/components/PageHeader";

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <div className="pb-2">
      <PageHeader title={"Downloads"} />
      {children}
    </div>
  );
}
