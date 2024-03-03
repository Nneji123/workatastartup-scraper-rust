#![allow(dead_code)]
// CLASS NAMES FOR LOGIN
pub const LOGIN_BUTTON_XPATH: &str = "/html/body/header/nav/div[3]/div[2]/a";
pub const USERNAME_INPUT_XPATH: &str = r"//*[@id='ycid-input']";
pub const PASSWORD_INPUT_XPATH: &str = "//*[@id='password-input']";
pub const SUBMIT_BUTTON_XPATH: &str = "//*[@id='sign-in-card']/div[2]/div[8]/button/span[1]";

// CLASS NAMES FOR FOUNDERS
pub const FOUNDER_NAME_CLASS: &str = "mb-1.font-medium";
pub const FOUNDER_IMAGE_CLASS: &str = r"ml-2.mr-2.h-20.w-20.rounded-full.sm\:ml-5";
pub const FOUNDER_DESCRIPTION_CLASS_ONE: &str = r"sm\:text-md.text-sm";
pub const FOUNDER_DESCRIPTION_CLASS_TWO: &str = r"sm\:text-md.w-full.text-sm";
pub const FOUNDER_LINKEDIN_CLASS: &str = "fa.fa-linkedin.ml-4.p-1.text-blue-600";

// CLASS NAMES FOR COMPANY_DATA
pub const COMPANY_IMAGE_CLASS: &str = r"mt-2.sm\:w-28";
pub const COMPANY_NAME_CLASS: &str = r"company-name.hover\:underline";
pub const COMPANY_DESCRIPTION_CLASS_ONE: &str =
    r"sm\:text-md.prose.col-span-11.mx-5.max-w-none.text-sm";
pub const COMPANY_DESCRIPTION_CLASS_TWO: &str = "mt-3.text-gray-700";
pub const COMPANY_TAGS_CLASS: &str = "detail-label.text-sm";
pub const COMPANY_JOB_CLASS: &str = r"font-medium.text-gray-900.hover\:underline.sm\:text-lg";
pub const COMPANY_SOCIAL_CLASS: &str = "text-blue-600.ellipsis";

// CLASS NAMES FOR JOB_DATA
pub const JOB_TITLE_CLASS: &str = r"company-name.text-2xl.font-bold";
pub const JOB_TAGS_CLASS: &str = r"company-details.my-2.flex.flex-wrap.md\:my-0";
pub const SALARY_RANGE_CLASS: &str = "text-gray-500.my-2";
pub const JOB_DESCRIPTION_CLASS: &str = "prose";
