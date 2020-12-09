<template>
    <div class="px-4 py-8">
        <div>
            <h3 class="text-lg leading-6 font-medium text-gray-900">
                User Information
            </h3>
            <p class="mt-1 max-w-2xl text-sm text-gray-500">
                Personal details for the current user.
            </p>
        </div>
        <div v-if="user" class="mt-5 border-t border-gray-200">
            <dl class="sm:divide-y sm:divide-gray-200">
                <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4">
                    <dt class="text-sm font-medium text-gray-500">Full name</dt>
                    <dd
                        class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2"
                    >
                        {{ user.full_name }}
                    </dd>
                </div>
                <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4">
                    <dt class="text-sm font-medium text-gray-500">
                        Email address
                    </dt>
                    <dd
                        class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2"
                    >
                        {{ user.email }}
                    </dd>
                </div>
                <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4">
                    <dt class="text-sm font-medium text-gray-500">Joined</dt>
                    <dd
                        class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2"
                    >
                        {{ new Date(user.created_at).toLocaleString() }}
                    </dd>
                </div>
                <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4">
                    <dt class="text-sm font-medium text-gray-500">
                        Last Updated
                    </dt>
                    <dd
                        class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2"
                    >
                        {{ new Date(user.updated_at).toLocaleString() }}
                    </dd>
                </div>
                <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4">
                    <dt class="text-sm font-medium text-gray-500">About</dt>
                    <dd
                        class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2"
                    >
                        Fugiat ipsum ipsum deserunt culpa aute sint do nostrud
                        anim incididunt cillum culpa consequat. Excepteur qui
                        ipsum aliquip consequat sint. Sit id mollit nulla mollit
                        nostrud in ea officia proident. Irure nostrud pariatur
                        mollit ad adipisicing reprehenderit deserunt qui eu.
                    </dd>
                </div>
            </dl>
        </div>
    </div>
</template>

<script>
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import http from '../services/http.js'

export default {
    setup() {
        const user = ref(null)
        const router = useRouter()

        onMounted(async () => {
            try {
                let { data } = await http.get('me')
                user.value = data
            } catch (e) {
                if (e.status === 401) {
                    router.push({ name: 'login' })
                    return
                }
                console.error(e)
            }
        })

        return { user }
    },
}
</script>
