// Public surface of the auth feature. Consumers import from "@/features/auth"
// and never reach into the feature's internals — that encapsulation is the
// whole point of the feature-first layout. Files WITHIN the feature import
// each other relatively (./AuthContext, ../useAuth) to avoid a barrel cycle.
export { AuthProvider } from "./AuthContext";
export { useAuth } from "./useAuth";
export { RequireAuth, RequireGuest } from "./Guards";
export { LoginPage } from "./pages/LoginPage";
export { RegisterPage } from "./pages/RegisterPage";
export type { TokenResponse, UserResponse } from "./api";
