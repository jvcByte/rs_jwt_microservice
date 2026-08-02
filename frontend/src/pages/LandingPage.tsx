import { Link } from "react-router";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { ShieldCheck, Zap, RefreshCw, Lock } from "lucide-react";

const features = [
  {
    icon: ShieldCheck,
    title: "Argon2id passwords",
    description: "Industry-standard password hashing. No MD5, no shortcuts.",
  },
  {
    icon: Zap,
    title: "Short-lived JWTs",
    description: "Access tokens expire in minutes. Refresh tokens rotate on every use.",
  },
  {
    icon: RefreshCw,
    title: "Token rotation",
    description: "Refresh token reuse detection built in. Sessions stay secure.",
  },
  {
    icon: Lock,
    title: "Instant revocation",
    description: "Token versioning lets you invalidate all sessions immediately.",
  },
];

export function LandingPage() {
  return (
    <div className="flex flex-col">
      {/* Hero */}
      <section className="flex flex-col items-center justify-center gap-6 px-4 py-24 text-center">
        <Badge variant="outline" className="rounded-full">
          Rust · Actix-Web · React · Tauri
        </Badge>
        <h1 className="max-w-2xl text-4xl font-bold tracking-tight sm:text-5xl">
          A production-ready app template that takes security seriously
        </h1>
        <p className="max-w-xl text-lg text-muted-foreground">
          Full-stack monorepo with JWT auth, refresh token rotation, and a cross-platform
          frontend — ready to clone and build on.
        </p>
        <div className="flex gap-3">
          <Button size="lg" asChild>
            <Link to="/register">Get started free</Link>
          </Button>
          <Button size="lg" variant="outline" asChild>
            <Link to="/login">Sign in</Link>
          </Button>
        </div>
      </section>

      <Separator />

      {/* Features */}
      <section className="py-20 px-4">
        <div className="container mx-auto">
          <h2 className="mb-12 text-center text-2xl font-semibold tracking-tight">
            Built with the right defaults
          </h2>
          <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-4">
            {features.map(({ icon: Icon, title, description }) => (
              <Card key={title}>
                <CardHeader className="pb-2">
                  <Icon size={20} className="text-muted-foreground mb-1" />
                  <CardTitle className="text-base">{title}</CardTitle>
                </CardHeader>
                <CardContent>
                  <CardDescription>{description}</CardDescription>
                </CardContent>
              </Card>
            ))}
          </div>
        </div>
      </section>

      <Separator />

      {/* CTA */}
      <section className="py-20 px-4 text-center">
        <h2 className="text-2xl font-semibold tracking-tight mb-4">
          Ready to build something?
        </h2>
        <p className="text-muted-foreground mb-8">
          Clone the repo, set your env vars, and ship.
        </p>
        <Button asChild>
          <Link to="/register">Create an account</Link>
        </Button>
      </section>
    </div>
  );
}
