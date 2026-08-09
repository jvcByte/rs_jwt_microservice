import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/shared/components/ui/card";
import { Badge } from "@/shared/components/ui/badge";
import { Avatar, AvatarFallback } from "@/shared/components/ui/avatar";
import { useAuth } from "@/features/auth";
import { platform } from "@/shared/lib/platform";
import { ShieldCheck, Activity } from "lucide-react";

export function DashboardPage() {
  const { user } = useAuth();

  const initials = user?.name
    ?.split(" ")
    .map((w) => w[0])
    .join("")
    .toUpperCase()
    .slice(0, 2) ?? "??";

  return (
    <div className="p-6 max-w-2xl mx-auto space-y-6">
      {/* Greeting */}
      <div className="flex items-center gap-4">
        <Avatar className="h-12 w-12">
          <AvatarFallback>{initials}</AvatarFallback>
        </Avatar>
        <div>
          <h1 className="text-xl font-semibold">
            Hello, {user?.name ?? "there"} 👋
          </h1>
          <p className="text-sm text-muted-foreground">{user?.email}</p>
        </div>
      </div>

      {/* Status cards */}
      <div className="grid gap-4 sm:grid-cols-2">
        <Card>
          <CardHeader className="pb-2 flex-row items-center gap-2 space-y-0">
            <ShieldCheck size={16} className="text-muted-foreground" />
            <CardTitle className="text-sm">Session</CardTitle>
          </CardHeader>
          <CardContent>
            <Badge variant="outline" className="text-green-600 border-green-300">
              Active
            </Badge>
            <CardDescription className="mt-1.5">
              JWT + rotating refresh token
            </CardDescription>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2 flex-row items-center gap-2 space-y-0">
            <Activity size={16} className="text-muted-foreground" />
            <CardTitle className="text-sm">Platform</CardTitle>
          </CardHeader>
          <CardContent>
            <Badge variant="secondary" className="capitalize">
              {platform}
            </Badge>
            <CardDescription className="mt-1.5">
              Same codebase, adapted UI
            </CardDescription>
          </CardContent>
        </Card>
      </div>

      {/* Placeholder content area */}
      <Card>
        <CardHeader>
          <CardTitle>Your content goes here</CardTitle>
          <CardDescription>
            This template ships with auth wired up. Replace this with your app's features.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="h-32 rounded-md border border-dashed flex items-center justify-center text-sm text-muted-foreground">
            Add your feature here
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
