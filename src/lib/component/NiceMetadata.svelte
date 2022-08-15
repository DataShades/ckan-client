<script lang="ts">
    import { Table } from "sveltestrap";

    export let metadata = {};
    let rows = Object.entries(metadata);
</script>

<Table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Value</th>
        </tr>
    </thead>
    <tbody>
        {#each rows as row}
            <tr>
                <td>
                    {row[0]}
                </td>
                <td> {row[1]} </td>
            </tr>
        {/each}
    </tbody>
</Table>
