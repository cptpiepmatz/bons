import type { CreateSpendingRequest } from "../../bindings/CreateSpendingRequest.ts";
import type { CreateUserRequest } from "../../bindings/CreateUserRequest.ts";
import type { CreateUserResponse } from "../../bindings/CreateUserResponse.ts";
import type { LoginRequest } from "../../bindings/LoginRequest.ts";
import type { LoginResponse } from "../../bindings/LoginResponse.ts";
import type { SpendingResponse } from "../../bindings/SpendingResponse.ts";
import type { UserResponse } from "../../bindings/UserResponse.ts";
import type { Month } from "../models/month.ts";

export class ApiError extends Error {
  constructor(
    public readonly status: number,
    public readonly path: string,
    public readonly method: string,
  ) {
    super(`API request failed: ${method} ${path} (${status})`);
    this.name = "ApiError";
  }
}

export class Api {
  private static async request(
    path: string,
    body?: unknown,
    signal?: AbortSignal,
  ): Promise<Response> {
    const url = `/api${path}`;
    const method = body === undefined ? "GET" : "POST";
    const response = await fetch(url, {
      method,
      credentials: "same-origin",
      signal,
      ...(body === undefined ? {} : {
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body),
      }),
    });

    if (!response.ok) throw new ApiError(response.status, url, method);
    return response;
  }

  static async getCurrency(): Promise<string> {
    return (await Api.request("/currency")).text();
  }

  static async login(body: LoginRequest): Promise<LoginResponse> {
    return (await Api.request("/login", body)).json();
  }

  static async getCurrentUser(): Promise<UserResponse> {
    return (await Api.request("/me")).json();
  }

  static async logout(): Promise<void> {
    await Api.request("/logout", {});
  }

  static async getSpendingForMonth(
    year: number,
    month: Month,
    signal?: AbortSignal,
  ): Promise<SpendingResponse[]> {
    return (await Api.request(
      `/spending/${year}/${month}`,
      undefined,
      signal,
    ))
      .json();
  }

  static async createSpending(
    body: CreateSpendingRequest,
  ): Promise<SpendingResponse> {
    return (await Api.request("/spending", body)).json();
  }

  static async createUser(
    body: CreateUserRequest,
  ): Promise<CreateUserResponse> {
    return (await Api.request("/users", body)).json();
  }
}
