import Footer from "./footer";
import "./globals.css";
import Nav from "./nav";
import { Inter } from "next/font/google";

export const metadata = {
  title: "jackpalaia.com",
};

const inter = Inter({
  subsets: ["latin"],
  display: "swap",
});

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" className={inter.className}>
      <body>
        <Nav />
        {children}
        <Footer />
      </body>
    </html>
  )
}
