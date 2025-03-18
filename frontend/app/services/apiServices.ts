import { getAccessToken } from "../lib/actions";

const access_token = await getAccessToken();
const apiServices = {
    getNoheader: async function (url: string): Promise<any> {
        return new Promise((resolve, reject) => {
            fetch(process.env.NEXT_PUBLIC_API_HOST + url, {
                method:'GET',
                headers: {
                    'Authorization': `Bearer ${access_token}`,
                }
            }).then(response => response.json())
                .then((json) => {
                    resolve(json);
                }).catch((error => {
                    reject(error);
            }))
        })
    },
    get: async function (url: string): Promise<any> {
        return new Promise((resolve, reject) => {
            fetch(process.env.NEXT_PUBLIC_API_HOST + url, {
                method:'GET',
                headers: {
                    'Accept':'application/json',
                    'Content-Type': 'applicaiton/json',
                    'Authorization': `Bearer ${access_token}`,
                }
            }).then(response => response.json())
                .then((json) => {
                    resolve(json);
                }).catch((error => {
                    reject(error);
            }))
        })
    },
    getWithoutToken: async function (url:string): Promise<any> {
        return new Promise((resolve, reject) => {
            fetch(`${process.env.NEXT_PUBLIC_API_HOST + url}`, {
                method: 'GET',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json'
                }
            }).then(response => response.json()).then((json) => {
                resolve(json);
            }).catch((error) => {
                reject(error);
            })
        })
    },
    post: async function (url: string, data: any, formData: boolean = false): Promise<any> {
        return new Promise((resolve, reject) => {
            fetch(`${process.env.NEXT_PUBLIC_API_HOST + url}`, {
                method: 'POST',
                body: data,
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${access_token}`
                }
            }

            ).then(response => response.json()).then((json) => {
                resolve(json);
            }).catch((error => {
                reject(error);
            }))
        })
    },
    postNoheaders: async function (url: string, data: any, formData: boolean = false): Promise<any> {
        return new Promise((resolve, reject) => {
            fetch(`${process.env.NEXT_PUBLIC_API_HOST + url}`, {
                method: 'POST',
                body: data,
                headers: {
                    'Authorization': `Bearer ${access_token}`
                }
            }

            ).then(response => response.json()).then((json) => {
                resolve(json);
            }).catch((error => {
                reject(error);
            }))
        })
    },
    postWithoutToken: async function (url: string, data: any): Promise<any>{
        return new Promise((resolve, reject) => {
            fetch(`${process.env.NEXT_PUBLIC_API_HOST + url}`, {
                method: 'POST',
                body: data,
                headers: {
                    'Accept': 'application/json',
                    'Content-Type':'application/json'
                }
            }).then(response => response.json()).then((json) => {
                resolve(json);
            }).catch((error) => {
                reject(error);
            })
        })
    }
};

export default apiServices