export interface GetTokenResponse {
    token?: string
}

export const getToken = (): Promise<GetTokenResponse> => fetch('/token', { method: 'GET' }).then(response => response.json())