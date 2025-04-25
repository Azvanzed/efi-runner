use std::path::Path;

pub mod vmrun;

#[derive(Clone)]
pub struct VMWare {
    vmx_path: String,
    password: Option<String>,
}

impl VMWare {
    pub fn new(vmx_path: &Path, password: Option<String>) -> anything::Result<Self> {
        let vmx_path = vmx_path.to_str().ok_or("Invalid VMX path")?;

        if !Path::new(vmx_path).exists() {
            return Err(format!("VMX path {} does not exist", vmx_path).into());
        }

        Ok(Self {
            vmx_path: vmx_path.to_string(),
            password,
        })
    }
}
