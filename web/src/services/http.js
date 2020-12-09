const API_ORIGIN = import.meta.env.VITE_API_ORIGIN

export async function get(url) {
    url = url.replace(/^\//, '')
    let res = await fetch(`http://${API_ORIGIN}/${url}`, {
        mode: 'cors',
        credentials: 'include',
        headers: {
            Accept: 'application/json',
            Cache: 'no-cache',
        },
    })
    if (res.headers.get('Content-Type') === 'application/json')
        res.data = await res.json()

    if (!res.ok) throw res

    return res
}

export async function post(url, json) {
    url = url.replace(/^\//, '')
    let res = await fetch(`http://${API_ORIGIN}/${url}`, {
        mode: 'cors',
        method: 'POST',
        credentials: 'include',
        body: JSON.stringify(json),
        headers: {
            Accept: 'application/json',
            Cache: 'no-cache',
            'Content-Type': 'application/json',
        },
    })
    if (res.headers.get('Content-Type') === 'application/json')
        res.data = await res.json()

    if (!res.ok) throw res

    return res
}

export default {
    get,
    post,
}
