export interface TokenResponse {
    token: string
    username: string
    display_name: string
    id: number
    avatar_url: string
}

export const getToken = (): Promise<TokenResponse|null> => fetch('/token', { method: 'GET' }).then(response => response.json())