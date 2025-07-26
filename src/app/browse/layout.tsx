import { PageHeader } from "@/components/PageHeader";

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <>
      <PageHeader title={"Browse"} />
      <div className="p-2">{children}</div>
    </>
  );
}
