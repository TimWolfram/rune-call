// Composables
import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    component: () => import('@/layouts/default/Default.vue'),
    children: [
      {
        path: '',
        name: 'Home',
        // route level code-splitting
        // this generates a separate chunk (about.[hash].js) for this route
        // which is lazy-loaded when the route is visited.
        component: () => import(/* webpackChunkName: "home" */ '@/views/Home.vue'),
      },
      {
        path: '/login',
        name: 'Login',
        component: () => import('@/views/Login.vue'),
      },
      {
        path: '/register',
        name: 'Register',
        component: () => import('@/views/Register.vue'),
      },

      {
        path: '/about',
        name: 'About',
        component: () => import('@/views/About.vue'),
      },
      
      {
        path: '/rooms',
        name: 'RoomsList',
        component: () => import('@/views/Rooms.vue'),
      },
      {
        path: '/rooms/create',
        name: 'CreateRoom',
        component: () => import('@/views/CreateRoom.vue'),
      },
      {
        path: '/rooms/:id',
        name: 'Room',
        component: () => import('@/views/RoomLobby.vue'),
        props: route => ({ roomId: Number(route.params.id) }),
      },
      {
        path: '/rooms/:id/game',
        name: 'Game',
        component: () => import('@/views/Game.vue'),
        props: route => ({ roomId: Number(route.params.id) }),
      }
    ],
  },
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
})

export default router
