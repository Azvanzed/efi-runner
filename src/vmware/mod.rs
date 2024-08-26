use std::path::PathBuf;

pub mod vmrun;

#[derive(Clone)]
pub struct VMWare {
    vmx_path: String,
}

impl VMWare {
    pub fn new(vmx_path: &PathBuf) -> anything::Result<Self> {
        let vmx_path = vmx_path.to_str().ok_or("Invalid VMX path")?;

        if !std::path::Path::new(vmx_path).exists() {
            return Err(format!("VMX path {} does not exist", vmx_path).into());
        }

        Ok(Self {
            vmx_path: vmx_path.to_string(),
        })
    }
}
