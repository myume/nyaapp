import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import "./globals.css";
import { SidebarProvider } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/AppSidebar";
import { ThemeProvider } from "@/components/providers/ThemeProvider";
import { DownloadsProvider } from "@/components/providers/DownloadsProvider";
import { Toaster } from "@/components/ui/sonner";
import { ReaderProvider } from "@/components/providers/ReaderProvider";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
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
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased`}
      >
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
