/** @type {import('next').NextConfig} */
const nextConfig = {
  // output: "export",
  rewrites: async () => {
    return {
      fallback: [
        {
          source: "/:path*",
          destination: `http://localhost:8080/:path*`,
        },
      ],
    };
  },
};

module.exports = nextConfig;