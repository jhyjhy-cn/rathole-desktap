<script setup lang="ts">
import { useConfigStore } from '../stores/config'
import { storeToRefs } from 'pinia'
import { computed, ref } from 'vue'

const store = useConfigStore()
const { currentConfig } = storeToRefs(store)

const services = computed(() => {
    const list: any[] = []
    if (currentConfig.value?.client?.services) {
        Object.entries(currentConfig.value.client.services).forEach(([name, svc]: [string, any]) => {
            list.push({ name, type: 'client', ...svc })
        })
    }
    if (currentConfig.value?.server?.services) {
        Object.entries(currentConfig.value.server.services).forEach(([name, svc]: [string, any]) => {
            list.push({ name, type: 'server', ...svc })
        })
    }
    return list
})

const dialogVisible = ref(false)
const editingService = ref<any>({})
const isNew = ref(false)

function editService(svc: any) {
    editingService.value = { ...svc }
    isNew.value = false
    dialogVisible.value = true
}

function addService() {
    editingService.value = { name: '', type: 'client', token: '', local_addr: '127.0.0.1:80' }
    isNew.value = true
    dialogVisible.value = true
}

function saveService() {
    // Update store config object (deep merge/replace)
    // This is a simplified implementation. Real one needs deep reactivity or utility.
    const cfg = JSON.parse(JSON.stringify(currentConfig.value || {}))
    
    if (editingService.value.type === 'client') {
        if (!cfg.client) cfg.client = {}
        if (!cfg.client.services) cfg.client.services = {}
        cfg.client.services[editingService.value.name] = {
            token: editingService.value.token,
            local_addr: editingService.value.local_addr
        }
    } else {
        if (!cfg.server) cfg.server = {}
        if (!cfg.server.services) cfg.server.services = {}
        cfg.server.services[editingService.value.name] = {
            token: editingService.value.token,
            bind_addr: editingService.value.bind_addr
        }
    }
    
    store.updateFromObject(cfg)
    dialogVisible.value = false
}
</script>

<template>
  <div class="proxy-page">
    <div class="header">
        <h2>{{ $t('proxy.title') }}</h2>
        <el-button type="primary" @click="addService">{{ $t('proxy.addService') }}</el-button>
    </div>
    
    <div class="service-list">
        <el-empty v-if="services.length === 0" :description="$t('proxy.noServices')" />
        <el-row :gutter="20">
            <el-col :span="12" v-for="svc in services" :key="svc.name" style="margin-bottom: 20px;">
                <el-card shadow="hover">
                    <template #header>
                        <div class="card-header">
                            <span>{{ svc.name }}</span>
                            <el-tag size="small">{{ svc.type }}</el-tag>
                        </div>
                    </template>
                    <div class="svc-details">
                        <p v-if="svc.local_addr">{{ $t('proxy.local') }}: {{ svc.local_addr }}</p>
                        <p v-if="svc.bind_addr">{{ $t('proxy.bind') }}: {{ svc.bind_addr }}</p>
                        <p>{{ $t('proxy.token') }}: {{ svc.token ? '******' : $t('proxy.none') }}</p>
                    </div>
                    <div class="actions">
                        <el-button size="small" @click="editService(svc)">{{ $t('common.edit') }}</el-button>
                    </div>
                </el-card>
            </el-col>
        </el-row>
    </div>

    <el-dialog v-model="dialogVisible" :title="isNew ? $t('proxy.addService') : $t('proxy.editService')">
        <el-form :model="editingService" label-width="100px">
            <el-form-item :label="$t('proxy.name')">
                <el-input v-model="editingService.name" :disabled="!isNew" />
            </el-form-item>
            <el-form-item :label="$t('proxy.type')">
                <el-radio-group v-model="editingService.type" :disabled="!isNew">
                    <el-radio-button label="client">{{ $t('proxy.client') }}</el-radio-button>
                    <el-radio-button label="server">{{ $t('proxy.server') }}</el-radio-button>
                </el-radio-group>
            </el-form-item>
            <el-form-item :label="$t('proxy.token')">
                <el-input v-model="editingService.token" type="password" show-password />
            </el-form-item>
            <el-form-item :label="$t('proxy.address')" v-if="editingService.type === 'client'">
                <el-input v-model="editingService.local_addr" placeholder="127.0.0.1:80" />
            </el-form-item>
            <el-form-item :label="$t('proxy.bindAddr')" v-if="editingService.type === 'server'">
                <el-input v-model="editingService.bind_addr" placeholder="0.0.0.0:2333" />
            </el-form-item>
        </el-form>
        <template #footer>
            <el-button @click="dialogVisible = false">{{ $t('common.cancel') }}</el-button>
            <el-button type="primary" @click="saveService">{{ $t('common.confirm') }}</el-button>
        </template>
    </el-dialog>
  </div>
</template>

<style scoped>
.header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
}
.card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
}
.actions {
    margin-top: 10px;
    text-align: right;
}
</style>
