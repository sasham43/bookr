<script lang="ts">
    import { createEventDispatcher } from 'svelte'
    import { Alert, Button, Label, Input, Checkbox, Select } from 'flowbite-svelte'

    import { post }              from '../api'
    import { ageRanges, states } from '../constants'
    import { clearExpiredToken } from '../functions'
    import { authenticated }     from '../store'
    import type { NewContact }   from '../types'

    const dispatch = createEventDispatcher()

    let currentAction  = 'add' // TODO: ability to edit contacts
    let errorMessage   = ''
    let successMessage = ''

    let displayName: string
    let address: string | null
    let city: string
    let state: string
    let zipCode: string | null
    let ageRange: string = "unknown"
    let capacity: number | null
    let email: string | null
    let contactForm: string | null
    let isPrivate: boolean = false 

    async function submit() {
        if (currentAction === 'add') {
            await addNewContact()
        } else if (currentAction === 'edit') {
            // TODO: build ability for user to edit their contacts
        }
    }

    async function addNewContact() {
        errorMessage = ''

        const contact: NewContact = {
            displayName,
            address,
            city,
            state,
            zipCode,
            ageRange,
            capacity: 10,
            email,
            contactForm,
            isPrivate
        }

        const res = await post("/user/add-contact", contact)

        if (res.status === 200) {
            successMessage = "Contact has successfully added and will appear once approved."

            setTimeout(() => {
                dispatch('close')
            }, 2000)
        }

        if (res.status === 401) {
            // Shouldn't happen but user could try faking token
            // Token might also expire
            clearExpiredToken()
            authenticated.update(() => false)
            errorMessage = "Please log in again."
        }

        if (res.status === 400) {
            // shouldn't happen. input validators and tests on API
            // TODO: log content if it does, create report
            errorMessage = "There was an error adding the contact. Please try again."
        }

        if (res.status === 500) {
            errorMessage = "There was an error adding the contact. Please try again."
        }
    }
</script>

<form class="flex flex-col space-y-6" on:submit|preventDefault={submit}>
    <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Add new contact</h3>
    {#if errorMessage}
        <Alert border color="red">
            <svg slot="icon" aria-hidden="true" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path></svg>
            <span class="font-medium">Error</span> { errorMessage }
        </Alert>
    {/if}
    {#if successMessage}
        <Alert border color="blue">
            <svg slot="icon" aria-hidden="true" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path></svg>
            <span class="font-medium">Success</span> { successMessage }
        </Alert>
    {/if} 
    <Label class="space-y-2">
        <span>Display Name</span>
        <Input type="text" name="displayName" placeholder="" bind:value={displayName} required />
    </Label>
    <Label class="space-y-2">
        <span>Address</span>
        <Input type="text" name="address" placeholder="666 aska punk ave" bind:value={address}/>
    </Label>
    <Label class="space-y-2">
        <span>City</span>
        <Input type="text" name="city" placeholder="" bind:value={city} required/>
    </Label>
    <Label>State/Province
        <Select class="mt-2" items={states} bind:value={state} required/>
    </Label>
    <Label class="space-y-2">
        <span>Zip code</span>
        <Input type="text" name="zipCode" placeholder="" bind:value={zipCode} />
    </Label>
    <Label>Age range
        <Select class="mt-2" items={ageRanges} bind:value={ageRange} required/>
    </Label>
    <Label class="space-y-2">
        <span>Capacity</span>
        <Input type="number" name="capacity" placeholder="" bind:value={capacity} />
    </Label>
    <Label class="space-y-2">
        <span>Email</span>
        <Input type="email" name="email" placeholder="" bind:value={email} />
    </Label>
    <Label class="space-y-2">
        <span>Contact link</span>
        <Input type="text" name="contactForm" placeholder="" bind:value={contactForm} />
    </Label>
    <Checkbox checked={isPrivate}>Contact is private</Checkbox>

    <Button type="submit" class="w-full1">Add contact</Button>
</form>