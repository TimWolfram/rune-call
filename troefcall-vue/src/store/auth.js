import { defineStore } from "pinia";
import { del, post, put } from "@/requests";
import Cookies from "js-cookie";

const LOGIN_ENDPOINT = 'login';

export const useAuthStore = defineStore({
    id: 'auth',
    state: () => ({
        user: getUserFromLocalStorage(),
    }),
    getters: {
        loggedIn: (state) => {
            // checks if user is logged in locally
            console.log('Checking loggedIn: ' + JSON.stringify(state.user, null, 2));
            return state.user != null;
        },

        confirmedLoggedIn: (state) => {
            //confirms login with backend server
            console.log('Checking confirmedLoggedIn: ' + JSON.stringify(state.user, null, 2));
            state.checkLogin().then((response) => {
                console.log('confirmedLoggedIn: ' + JSON.stringify(response.data, null, 2));
                return response.data != null;
            })
                .catch((error) => {
                    console.log('Error while checking confirmedLoggedIn: ' + error);
                    return false;
                });
        },

        getDisplayName: (state) => {
            const user = state.user;
            console.log('Getting display name from local storage:\n' + JSON.stringify(user, null, 2));
            return user?.nickname ? user.nickname : (user?.username ? user.username : '')
        },
        getUsername: (state) => {
            return state.user?.username;
        },

        isInAnyRoom: async (state) => {
            let inRoom = await state.checkLogin()
                .then((response) => {
                    console.log('Checked isInRoom: ' + JSON.stringify(response.data, null, 2));
                    return response.data?.current_room != null;
                })
                .catch((error) => {
                    console.log('Error while checking isInRoom: ' + error);
                    return false;
                });
            console.log('Checked isInRoom: ' + inRoom);
            return inRoom;
        },

        getRoomId: (state) => {
            console.log('Getting room id from local storage:\n' + JSON.stringify(state.user, null, 2));
            let room = state.user?.current_room;
            if (room != null && typeof room !== 'number') {
                console.error('Room must be null or a number!');
                return null;
            }
            return room;
        },
    },
    actions: {
        async checkLogin() {
            return post(LOGIN_ENDPOINT) // login on server without credentials will check token in cookies
                .then((response) => {
                    console.log(`logged in user: \n${JSON.stringify(response.data, null, 2)}`);
                    // update pinia state
                    this.user = response.data;
                    localStorage.setItem('user', JSON.stringify(response.data));
                    return response;
                })
                .catch((error) => {
                    console.log('Error while checking login: ' + error.response.data);
                    this.user = null;
                    localStorage.removeItem('user');
                    throw error;
                });
        },

        async login(username, password) {
            return post(LOGIN_ENDPOINT, { username, password })
                .then((response) => {
                    // update pinia state
                    console.log(`logged in user: \n${JSON.stringify(response.data, null, 2)}`);
                    this.user = response.data;
                    localStorage.setItem('user', JSON.stringify(response.data));
                    return response;
                })
                .catch((error) => {
                    console.error('Error while logging in: ' + error);
                    this.user = null;
                    localStorage.removeItem('user');
                    throw error;
                });
        },

        async register(username, password) {
            return post(LOGIN_ENDPOINT + '/register', { username, password })
                .then((response) => {
                    // update pinia state
                    this.user = response.data;
                    localStorage.setItem('user', JSON.stringify(response.data));
                    return response;
                })
                .catch((error) => {
                    console.log('Error while registering: ' + error);
                    throw error;
                });
        },

        async setDisplayName(nickname) {
            return put(LOGIN_ENDPOINT + '/nickname', nickname)
                .then((response) => {
                    // update pinia state
                    this.user.nickname = nickname;
                    localStorage.setItem('user', JSON.stringify(this.user));
                    return response;
                })
                .catch((error) => {
                    console.log('Error while setting nickname: ' + error);
                    throw error;
                });
        },

        async logout() {
            del(LOGIN_ENDPOINT).then((response) => {
                console.log('Succesfully logged out user!');
            }
            ).catch((error) => {
                console.log('Error while logging out: ' + error);
                throw error;
            });
            // remove local storage regardless of backend response
            this.user = null;
            localStorage.removeItem('user');
            // remove cookies (set by backend)
            Cookies.remove('login_user_token', {domain: '127.0.0.1', sameSite: 'none', secure: true});
        },

        async setUser(user) {
            console.log('Setting user: ' + JSON.stringify(user, null, 2));
            this.user = user;
            localStorage.setItem('user', JSON.stringify(user));
        },

        async setCurrentRoom(room) {
            //check if null or number
            if (room != null && typeof room !== 'number') {
                throw new Error('Room must be null or a number!');
            }
            console.log('Setting current room: ' + room);
            this.user.current_room = room;
            localStorage.setItem('user', JSON.stringify(this.user));
        }
    },
});


function getUserFromLocalStorage() {
    let user = localStorage.getItem('user');
    if (user === null) {
        return null;
    }
    try {
        return JSON.parse(user);
    }
    catch (error) {
        console.error('Error while parsing user from local storage: ' + error);
        return null;
    }
}
