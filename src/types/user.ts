

export type SendVerificationCodeResult = {
    statusCode: number,
    comments: string,
    data: boolean | null
}

export type GeneratePhotoCodeResult = {
    statusCode: number,
    comments: string,
    data: {
        baseCode: string
    }
}

export type LoginResult = {
    statusCode: number,
    comments: string,
    data: {
        accessToken: string,
        refreshToken: string,
    }
}

export type UserProfileResult = {
    statusCode: number,
    comments: string,
    data: {
        nickname: string,
        avatar: string,
    }
}
