import axios from "axios";
import type { CandidateLogin } from "src/stores/candidate";
import { API_URL, errorHandler } from ".";


export async function apiLogin(data: CandidateLogin): Promise<boolean> {
    axios.post(API_URL + '/candidate/login', data, {withCredentials: true}).then((res) => {
        console.log(res);
        return res.status === 200;
    }).catch((err) => {
        throw errorHandler(err, "Login failed");
    });
    return false;
}

// TODO
export async function apiLogout(): Promise<boolean> {
    try {
        const res = await axios.get(API_URL + '/candidate/logout', {
            withCredentials: true
        });
        return res.status === 200;
    } catch (error) {
        return false;
    }
}