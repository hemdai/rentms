


const apiServices =  {
    get: async function (url: string): Promise<any> {
        url = process.env.NEXT_PUBLIC_API_HOST + url;
        console.log('url', url)

        return new Promise((resolve, reject) => {
            fetch(url, {
                method:'GET',
                headers: {
                    'Accept':'application/json',
                    'Content-Type': 'applicaiton/json',
                    'Authorization': `Bearer ${process.env.TOKEN}`,
                }
            }).then(response => response.json())
                .then((json) => {
                    resolve(json);
                }).catch((error => {
                    reject(error);
            }))
        })
    },
    post: async function(url: string, data: any): Promise<any> {
        return new Promise((resolve, reject) => {
            fetch(`${process.env.NEXT_PUBLIC_API_HOST + url}`, {
                method: 'POST',
                body: data,
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json',
                    'Authorization': `Bearere ${process.env.TOKEN}`
                }
            }

            ).then(response => response.json()).then((json) => {
                resolve(json);
            }).catch((error => {
                reject(error);
            }))
        })
    }
};

export default apiServices