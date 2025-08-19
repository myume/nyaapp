import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import { SidebarProvider } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/AppSidebar";
import { ThemeProvider } from "@/components/providers/ThemeProvider";
import { DownloadsProvider } from "@/components/providers/DownloadsProvider";
import { Toaster } from "@/components/ui/sonner";
import { ReaderProvider } from "@/components/providers/ReaderProvider";

const inter = Inter({
  variable: "--font-inter",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Nyaapp",
  description: "The Nyaa App",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body className={`${inter.variable} antialiased`}>
        <ThemeProvider
          attribute="class"
          defaultTheme="system"
          enableSystem
          disableTransitionOnChange
        >
          <DownloadsProvider>
            <ReaderProvider>
              <SidebarProvider>
                <AppSidebar />
                <main className="w-full h-full">{children}</main>
                <Toaster />
              </SidebarProvider>
            </ReaderProvider>
          </DownloadsProvider>
        </ThemeProvider>
      </body>
    </html>
  );
}
