<script lang="ts">
    import ov1_1 from "../../assets/ov1-1.png";
    import ov1_2 from "../../assets/ov1-2.png";
    import ov2_1 from "../../assets/ov2-1.png";
    import ov2_2 from "../../assets/ov2-2.png";
    import ov2_3 from "../../assets/ov2-3.png";
    import ov2_4 from "../../assets/ov2-4.png";
    import ov2_5 from "../../assets/ov2-5.png";
    import ov3_1 from "../../assets/ov3-1.png";
    import ov3_2 from "../../assets/ov3-2.png";
    import ov3_3 from "../../assets/ov3-3.png";
    import ov3_4 from "../../assets/ov3-4.png";
    import ov3_5 from "../../assets/ov3-5.png";

    import {
        Accordion,
        AccordionItem,
        Alert,
        Button,
        Icon,
        Offcanvas,
        Image,
    } from "sveltestrap";

    import { Manual } from "../../services";
    export let section: number | null = null;
    const toggle = () => Manual.toggle(section);
</script>

<div class="manual-wrapper">
    <Button color="light" class="manual-toggle" on:click={toggle}>
        <Icon name="question-square-fill" />
    </Button>
    <Offcanvas
        style="width: 90%"
        container="inline"
        isOpen={$Manual.open}
        {toggle}
        placement="start"
        header="Overview"
    >
        <div class="manual-restricted-width">
            <Alert color="primary">
                <p>
                    <strong>
                        This tool has been created to improve the utility of the
                        NSW Flood Data Portal
                    </strong> by allowing users to upload datasets in bulk to a Flood
                    Project that has already been created in the portal
                </p>
                <p>
                    Read the information about each step under the sub headings
                    below to ensure datasets are uploaded correctly. A user
                    guide can also be accessed via this link
                    <Button
                        color="light"
                        class="manual-toggle manual-toggle-ref"
                        on:click={toggle}
                    >
                        <Icon name="question-square-fill" />
                    </Button>
                </p>
            </Alert>
            <p>To upload datasets you should make 3 simple steps.</p>

            {#if $Manual.open}
                <div>
                    <Accordion>
                        <AccordionItem active={$Manual.active === 1}>
                            <span slot="header">
                                <span class="step-circle">1</span>
                                Select a project
                            </span>
                            <p>
                                Select a project that has been created in the
                                Flood Data Portal. You can select a project in 2
                                ways:
                            </p>
                            <p>
                                <strong>
                                    1. Type project name within the search field
                                </strong>
                                <Image fluid class="w-100" src={ov1_1} />
                            </p>
                            <p>
                                <strong>
                                    2. Select a project from the list.
                                </strong>
                            </p>
                            <p>
                                You can select only 1 project per upload.
                                <Image fluid class="w-100" src={ov1_2} />
                            </p>
                        </AccordionItem>
                        <AccordionItem active={$Manual.active === 2}>
                            <span slot="header">
                                <span class="step-circle">2</span>
                                Select a source
                            </span>
                            <p>
                                The source - these are files related to uploaded
                                datasets. You should <strong>
                                    select a folder, where dataset folders are
                                    located.
                                </strong>
                            </p>
                            <p>
                                <strong>
                                    1. Press “Browse” button, and select a
                                    folder with datasets.
                                </strong>
                                <Image fluid class="w-100" src={ov2_1} />
                            </p>
                            <p>
                                How is the structure of your files should be:
                                <Image fluid class="w-100" src={ov2_2} />
                            </p>
                            <p>
                                The example of the path to a folder with
                                datasets based on current structure and naming.
                                Of course, you can name them on your own.
                                <Image fluid class="w-100" src={ov2_3} />
                            </p>
                            <p><strong> 2. Add Master Metadata </strong></p>
                            <Alert color="primary">
                                The most important metadata to create.
                            </Alert>
                            <p>
                                If your dataset folders already have the
                                metadata, you will see it below the browse
                                input. Otherwise, you will show a message, where
                                the app proposes you create metadata
                                automatically. An automatically created metadata
                                must be edited this or in the next step.
                            </p>
                            <p>
                                <strong>Case 1:</strong>
                                Metadata is already created.
                                <Image fluid class="w-100" src={ov2_4} />
                            </p>
                            <p>
                                <strong> Case 2: </strong>
                                Metadata is not created yet.
                                <Image fluid class="w-100" src={ov2_5} />
                            </p>
                        </AccordionItem>
                        <AccordionItem active={$Manual.active === 3}>
                            <span slot="header">
                                <span class="step-circle">3</span>
                                Validate datasets and upload
                            </span>
                            <p>
                                On this step you need to review files and
                                folders metadata and validate them.
                            </p>
                            <h5>Master metadata</h5>
                            <Alert color="primary">
                                The most important metadata to create.
                            </Alert>
                            <p>
                                Datasets and resources metadata will refer to it
                                if they will not have enough data.
                                <Image fluid class="w-100" src={ov3_1} />
                            </p>
                            <h5>
                                3.1. The anatomy of a dataset tile. General
                                overview
                            </h5>
                            <p>
                                <Image fluid class="w-100" src={ov3_2} />
                            </p>
                            <p>
                                1. <strong> The dataset tile </strong> that
                                contains all items for validating and reviewing
                                dataset data. <br />
                                2. <strong> The dataset title. </strong><br />
                                3.
                                <strong>
                                    The button that opens the dataset folder
                                </strong>
                                where all dataset files located. <br />
                                4. <strong> The dataset metadata tab </strong>
                                where you can look at the metadata and open a
                                file with it. <br />
                                5. <strong> Resources tab </strong> where you
                                can review all files and make some actions with
                                them. <br />
                                6. <strong> The validation errors tile </strong>
                                where all errors, warnings and success messages
                                about current dataset will display. <br />
                                7. <strong> The upload button. </strong> It will
                                activate only when validation will have
                                completed.
                                <br />
                                8. <strong> Validate button. </strong> Press
                                this button to validate the current dataset.
                                <br />
                            </p>
                            <h5>
                                3.2. The anatomy of a dataset tile. Dataset
                                metadata tab
                            </h5>
                            <p>
                                <strong> Case 1: </strong>
                                Metadata is already created.
                                <Image fluid class="w-100" src={ov3_3} />
                            </p>
                            <p>
                                1. <strong> The metadata of a dataset. </strong>
                                When you make changes in the editor on your PC
                                or Mac it will change here immediately. <br />
                                2. <strong> The button </strong> that will open the
                                metadata in editor installed on your PC or Mac.
                            </p>
                            <p>
                                <strong> Case 2: </strong>
                                Metadata is not created yet.
                                <Image fluid class="w-100" src={ov3_4} />
                            </p>
                            <p>
                                You can create create metadata automatically and
                                then edit it and validate.
                            </p>
                            <h5>
                                3.3. The anatomy of a dataset tile. Resource tab
                            </h5>
                            <p>
                                <Image fluid class="w-100" src={ov3_5} />
                            </p>
                            <p>
                                <strong> 1. The resource title. </strong><br />
                                <strong>
                                    2. The resource metadata tab.
                                </strong><br />
                                <strong>
                                    3. The button to open metadata
                                </strong>
                                file in your editor on PC or Mac.
                            </p>
                            <p>
                                Other elements work the same as described in the
                                previous steps.
                            </p>
                        </AccordionItem>
                    </Accordion>
                </div>
            {/if}

            <br />
            <p>
                <Button color="primary" on:click={toggle}>
                    Start uploading
                </Button>
            </p>
        </div>
    </Offcanvas>
</div>
