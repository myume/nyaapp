import { PageHeader } from "@/components/PageHeader";

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <>
      <PageHeader title={"Library"} />
      {children}
    </>
  );
}
