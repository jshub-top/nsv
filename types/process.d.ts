declare namespace NodeJS {
    export interface ProcessEnv {
        NSV_STATUS: "0"|"1"|"2"
        NSV_HOME: string
        NSV_TEMP_SCRIPT_NAME: string
        NSV_CURRENT_VERSION?: string

        PATH: string
        http_proxy: string
        https_proxy: string
        HTTP_PROXY: string
        HTTPS_PROXY: string
    }
}

