"use client";

import Image from "next/image";
import Modal from "./Modal";
import useAddPropertyModal from "@/app/hooks/useAddPropertyModal";
import CustomButton from "../forms/CustomButton";
import { ChangeEvent, useState } from "react";
import Categories from "../addproperty/Categories";
import SelectCountry, { SelectCountryValue } from "../forms/SelectCountry";
import apiServices from "@/app/services/apiServices";
import { useRouter } from "next/navigation";
const AddPropertyModal = () => {
    // States
    const [currentStep, setCurrentStep] = useState(1);
    const [dataCategory, setDataCategory] = useState('');
    const [dataTitle, setDataTitle] = useState('');
    const [dataDescription, setDataDescription] = useState('');
    const [dataPrice, setDataPrice] = useState('');
    const [dataBedrooms, setBedrooms] = useState('');
    const [dataBathrooms, setBathrooms] = useState('');
    const [dataGuest, setGuest] = useState('');
    const [dataCountry, setDataCountry] = useState<SelectCountryValue>();
    const [dataImage, setDataImage] = useState<File | null>(null);
    const [dataHouseNumber, setHouseNumber] = useState('');
    const [dataStreetName, setStreetName] = useState('');
    const [dataCity, setCity] = useState('');
    const [dataPostalCode, setPostalCode] = useState('');
    //


    const addPropertyModal = useAddPropertyModal();
    const router = useRouter();

    //
    // Set datas
    const setCategory = (category: string) => {
        setDataCategory(category)
    }

    // Set Image
    const setImage = (event: ChangeEvent<HTMLInputElement>) => {
        if (event.target.files && event.target.files.length > 0) {
            const tmpImage = event.target.files[0];
            setDataImage(tmpImage);
        }
    }
    // GetCountry 
    const getCountryId = async (country_name: string) => {
        return await apiServices.getNoheader(`/country/${country_name}`)
    }
    // Create Address
    const createAddress = async () => {
        if (dataCity &&
            dataCountry &&
            dataPostalCode &&
            dataHouseNumber &&
            dataStreetName
        ) {
            const country_key = await getCountryId(dataCountry.label).then(response => response.id).catch(error => console.log(error));
            if (country_key) {
                const postData = {
                    street: dataStreetName,
                    building_no: parseInt(dataHouseNumber),
                    postal_code: parseInt(dataPostalCode),
                    country_id: country_key
                }
                console.log(postData, "Post Data");
                return await apiServices.post('/address/create-address', JSON.stringify(postData)).then(response => response).catch(error => console.log(error));
            }
        } console.log("Error on adding address")
    }

    

 
    // Submit Form
    const submitForm = async () => {
        console.log('submitForm');
        if (
            dataTitle &&
            dataDescription &&
            dataPrice &&
            dataCountry &&
            dataImage &&
            dataCategory
        ) {
            const address_key = await createAddress().then(response => response.id).catch(error => console.log(error));
            const formData = new FormData();
            formData.append('category', dataCategory);
            formData.append('title', dataTitle);
            formData.append('description', dataDescription);
            formData.append('price_per_night', dataPrice);
            formData.append('bedroom', dataBedrooms);
            formData.append('bathroom', dataBathrooms);
            formData.append('guest', dataGuest);
            formData.append('address_id', address_key);
            formData.append('image', dataImage);

            const response = await apiServices.postNoheaders('/property/create-property', formData);
            if (response.id) {
                console.log('Success :-D');
                router.push('/myproperties');
                addPropertyModal.close();
            } else {
                console.log('Error submitting forms');
            }
        }
    }

    const content = (
        <>
            {currentStep == 1 ? (
                <>
                    <h2 className="mb-6 text-2xl"> Choose Category </h2>
                    <Categories
                        dataCategory={dataCategory}
                        setCategory={(category) => setCategory(category)}
                    />

                    <CustomButton label="Next" onClick={() => setCurrentStep(2)} />
                </>
            ) : currentStep == 2 ? (

                <>
                    <h2 className="mb-6 text-2xl"> Describe Your Place  </h2>

                    <div className="pt-3 pb-6 space-y-4">
                        <div className="flex flex-col space-y-2">
                            <label>Title</label>
                            <input
                                type="text"
                                value={dataTitle}
                                onChange={(e) => setDataTitle(e.target.value)}
                                className="w-full p-4 border border-gray-600 rounded-xl"
                            />
                        </div>
                    </div>

                    <div className="pt-3 pb-6 space-y-4">
                        <div className="flex flex-col space-y-2">
                            <label>Description</label>
                            <textarea

                                value={dataDescription}
                                onChange={(e) => setDataDescription(e.target.value)}
                                className="w-full h-[200px] p-4 border border-gray-600 rounded-xl"
                            />
                        </div>
                    </div>
                    <CustomButton label="Previous" className="mb-2 bg-black hover:bg-gray 800" onClick={() => setCurrentStep(1)} />
                    <CustomButton label="Next" onClick={() => setCurrentStep(3)} />
                </>
            ) : currentStep == 3 ? (
                <>
                    <h2 className="mb-6 text-2xl"> Details </h2>
                    <div className="pt-3 pb-6 space-y-4">
                        <div className="flex flex-col space-y-2">
                            <label>Bedrooms</label>
                            <input
                                type="text"
                                value={dataBedrooms}
                                onChange={(e) => setBedrooms(e.target.value)}
                                className="w-full p-4 border border-gray-600 rounded-xl"
                            />
                        </div>

                        <div className="flex flex-col space-y-2">
                            <label>Bathrooms</label>
                            <input
                                type="text"
                                value={dataBathrooms}
                                onChange={(e) => setBathrooms(e.target.value)}
                                className="w-full p-4 border border-gray-600 rounded-xl"
                            />
                        </div>

                        <div className="flex flex-col space-y-2">
                            <label>Maximum number of Guests</label>
                            <input
                                type="text"
                                value={dataGuest}
                                onChange={(e) => setGuest(e.target.value)}
                                className="w-full p-4 border border-gray-600 rounded-xl"
                            />
                        </div>

                        <div className="flex flex-col space-y-2">
                            <label>Price</label>
                            <input
                                type="text"
                                value={dataPrice}
                                onChange={(e) => setDataPrice(e.target.value)}
                                className="w-full p-4 border border-gray-600 rounded-xl"
                            />
                        </div>
                    </div>

                    <CustomButton label="Previous" className="mb-2 bg-black hover:bg-gray 800" onClick={() => setCurrentStep(2)} />
                    <CustomButton label="Next" onClick={() => setCurrentStep(4)} />
                </>

            ) : currentStep == 4 ? (
                <>

                    <h2 className="mb-6 text-2xl"> Location </h2>

                    <div className="pt-3 pb-6 space-y-4">
                        Country
                        <SelectCountry
                            value={dataCountry}
                            onChange={(value) => setDataCountry(value as SelectCountryValue)}
                        />
                        <div className="flex flex-col space-y-2">

                            <label>House Number</label>
                            <input
                                type="text"
                                value={dataHouseNumber}
                                onChange={(e) => setHouseNumber(e.target.value)}
                                className="w-full p-4 border border-gray-600 rounded-xl"
                            />
                        </div>
                        <div className="flex flex-col space-y-2">

                            <label>Street Name /Avenue</label>
                            <input
                                type="text"
                                value={dataStreetName}
                                onChange={(e) => setStreetName(e.target.value)}
                                className="w-full p-4 border border-gray-600 rounded-xl"
                            />
                        </div>
                        <div className="flex flex-col space-y-2">
                            <label>City</label>
                            <input
                                type="text"
                                value={dataCity}
                                onChange={(e) => setCity(e.target.value)}
                                className="w-full p-4 border border-gray-600 rounded-xl"
                            />
                        </div>
                        <div className="flex flex-col space-y-2">
                            <label>Postal Code</label>
                            <input
                                type="text"
                                value={dataPostalCode}
                                onChange={(e) => setPostalCode(e.target.value)}
                                className="w-full p-4 border border-gray-600 rounded-xl"
                            />
                        </div>
                    </div>

                    <CustomButton label="Previous" className="mb-2 bg-black hover:bg-gray 800" onClick={() => setCurrentStep(3)} />
                    <CustomButton label="Next" onClick={() => setCurrentStep(5)} />
                </>
            ) : (
                <>
                    <h2 className="mb-6 text-2xl"> Images </h2>
                    <div className="pt-3 pb-6 space-y-4">
                        <div className='py-4 px-6 bg-gray-600 text-white rounded-xl'>
                            <input
                                type="file"
                                accept="image/*"
                                onChange={setImage}
                            />
                        </div>
                        {dataImage && (
                            <div className="w-[200px] h-[150px] relative">
                                <Image
                                    fill
                                    alt="Upload image"
                                    src={URL.createObjectURL(dataImage)}
                                    className="w-full h-full object-cover rounded-xl"
                                />
                            </div>
                        )}
                    </div>
                    <CustomButton label="Previous" className="mb-2 bg-black hover:bg-gray 800" onClick={() => setCurrentStep(4)} />
                    <CustomButton label="Submit" onClick={submitForm} />
                </>


            )}
        </>
    );
    return (
        <>
            <Modal
                isOpen={addPropertyModal.isOpen}
                close={addPropertyModal.close}
                label="Add Property"
                content={content}
            />
        </>
    );
};
export default AddPropertyModal;
