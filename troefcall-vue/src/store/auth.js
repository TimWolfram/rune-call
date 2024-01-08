import { defineStore } from "pinia";
import { get, post } from "@/requests";
import Cookies from "js-cookie";

const LOGIN_ENDPOINT = 'login';

export const useAuthStore = defineStore({
    id: 'auth',
    state: () => ({
        user: null,
    }),
    getters: {
        loggedIn: (state) => {
            // checks if user is logged in locally
            return state.user != null;
        },
        confirmedLoggedIn: (state) => {
            //confirms login with backend server
            checkLogin();
            return state.user != null;
        },
        getDisplayName: (state) => {
            const user = state.user;
            user?.displayName ? user.displayName : (user?.username ? user.username : '')
        },
        getLightmodePreference: () => {
            return localStorage.getItem(LIGHTMODE) === 'true';
        }
    },
    actions: {
        checkLogin(){
            return get(LOGIN_ENDPOINT) // login on server without credentials will check token in cookies
            .then((response) => {
                // update pinia state
                this.user = response.data;
                return response;
            })
            .catch((error) => {
                console.log('Error while checking login: ' + error);
                this.user = null;
                throw error;
            });
        },
        login(username, password) {
            return get(LOGIN_ENDPOINT, { username, password })
            .then((response) => {
                // update pinia state
                this.user = response.data;
                return response;
            })
            .catch((error) => {
                console.log('Error while logging in: ' + error);
                this.user = null;
                throw error;
            });
        },
        register(username, password) {
            return post(LOGIN_ENDPOINT, { username, password })
                .then((response) => {
                    // update pinia state
                    this.user = response.data;
                    return response;
                })
                .catch((error) => {
                    console.log('Error while registering: ' + error);
                    this.user = null;
                    throw error;
                });
        },
        logout() {
            this.user = null;
        },
        setLightModePreference(lightMode) {
            localStorage.setItem(LIGHTMODE, lightMode);
        }
    },
});