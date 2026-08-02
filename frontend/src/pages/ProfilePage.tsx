import { useState, type FormEvent } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { useAuth } from "@/hooks/useAuth";
import { useNavigate } from "react-router";
import { toast } from "sonner";

export function ProfilePage() {
  const { user, logout } = useAuth();
  const navigate = useNavigate();
  const [name, setName] = useState(user?.name ?? "");

  const initials = user?.name
    ?.split(" ")
    .map((w) => w[0])
    .join("")
    .toUpperCase()
    .slice(0, 2) ?? "??";

  async function handleLogout() {
    await logout();
    toast.success("Signed out");
    navigate("/");
  }

  // Placeholder — wire to PUT /api/users/:id when ready
  function handleSave(e: FormEvent) {
    e.preventDefault();
    toast.info("Profile update coming soon");
  }

  return (
    <div className="p-6 max-w-lg mx-auto space-y-6">
      {/* Avatar + identity */}
      <div className="flex items-center gap-4">
        <Avatar className="h-16 w-16">
          <AvatarFallback className="text-lg">{initials}</AvatarFallback>
        </Avatar>
        <div>
          <p className="font-semibold">{user?.name}</p>
          <p className="text-sm text-muted-foreground">{user?.email}</p>
        </div>
      </div>

      {/* Edit form */}
      <Card>
        <form onSubmit={handleSave}>
          <CardHeader>
            <CardTitle className="text-base">Edit profile</CardTitle>
            <CardDescription>Update your display name</CardDescription>
          </CardHeader>
          <CardContent className="flex flex-col gap-4">
            <div className="grid gap-1.5">
              <Label htmlFor="name">Name</Label>
              <Input
                id="name"
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="Your name"
              />
            </div>
            <div className="grid gap-1.5">
              <Label htmlFor="email">Email</Label>
              <Input id="email" value={user?.email ?? ""} disabled />
            </div>
          </CardContent>
          <CardFooter>
            <Button type="submit" size="sm">Save changes</Button>
          </CardFooter>
        </form>
      </Card>

      <Separator />

      {/* Danger zone */}
      <Card>
        <CardHeader>
          <CardTitle className="text-base text-destructive">Sign out</CardTitle>
          <CardDescription>Revoke your session and return to the home screen</CardDescription>
        </CardHeader>
        <CardFooter>
          <Button variant="destructive" size="sm" onClick={handleLogout}>
            Sign out
          </Button>
        </CardFooter>
      </Card>
    </div>
  );
}
