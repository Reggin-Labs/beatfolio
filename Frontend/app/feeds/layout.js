export const metadata = {
  title: 'Beatfolio | Feeds',
  description:
    'A decentralized social media platform tailored exclusively for artists, musicians, and creators.',
};

export default function RootLayout({ children }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}